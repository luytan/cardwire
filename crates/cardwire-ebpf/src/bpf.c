#include <linux/bpf.h>
#include <linux/errno.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

#ifndef ENOENT
#define ENOENT 2
#endif

typedef unsigned int u32;
typedef unsigned char u8;

struct qstr {
    const unsigned char *name;
} __attribute__((preserve_access_index));

struct dentry {
    struct qstr d_name;
    struct dentry *d_parent;
} __attribute__((preserve_access_index));

struct path {
    struct dentry *dentry;
} __attribute__((preserve_access_index));

struct file {
    struct path f_path;
} __attribute__((preserve_access_index));

char _license[] SEC("license") = "GPL";

/* Maps to store blocked IDs and PCI addresses */
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, u32);
    __type(value, u8);
} BLOCKED_IDS SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, char[16]);
    __type(value, u8);
} BLOCKED_PCI SEC(".maps");

/**
 * Helper to check if a name matches a blocked PCI address,
 * a renderD<id>, or a card<id>
 */
static __always_inline int is_blocked(char *name) {
    /* 1. Check for PCI address format: XXXX:XX:XX.X */
    if (name[4] == ':' && name[7] == ':' && name[10] == '.') {
        name[11] = '0';  /* Force function to 0 */
        name[12] = '\0'; /* Null terminate early */
        
        u8 *value = bpf_map_lookup_elem(&BLOCKED_PCI, name);
        return (value && *value == 1);
    }

    /* 2. Check for card<id> or renderD<id> */
    const char *p = NULL;
    if (name[0] == 'c' && name[1] == 'a' && name[2] == 'r' && name[3] == 'd') {
        p = name + 4;
    } else if (name[0] == 'r' && name[1] == 'e' && name[2] == 'n' && name[3] == 'd' && 
               name[4] == 'e' && name[5] == 'r' && name[6] == 'D') {
        p = name + 7;
    }

    /* If prefix matched, parse the numeric ID */
    if (p) {
        u32 id = 0;
        #pragma unroll
        for (int i = 0; i < 7; i++) {
            if (p[i] >= '0' && p[i] <= '9') {
                id = id * 10 + (p[i] - '0');
            } else {
                break;
            }
        }
        
        u8 *value = bpf_map_lookup_elem(&BLOCKED_IDS, &id);
        return (value && *value == 1);
    }

    return 0;
}

/* Blocks the actual opening of the file */
SEC("lsm/file_open")
int BPF_PROG(file_open, struct file *file) {
    struct dentry *dentry = BPF_CORE_READ(file, f_path.dentry);

    #pragma unroll
    for (int i = 0; i < 5; i++) {
        if (!dentry) break;

        const unsigned char *name_ptr = BPF_CORE_READ(dentry, d_name.name);
        if (!name_ptr) break;

        char buf[16] = {0};
        bpf_probe_read_kernel_str(&buf, sizeof(buf), name_ptr);
        /* i == 0 means we are looking at the exact file being opened */
        if (i == 0 && 
            buf[0] == 'p' && buf[1] == 'o' && buf[2] == 'w' && buf[3] == 'e' &&
            buf[4] == 'r' && buf[5] == '_' && buf[6] == 's' && buf[7] == 't' &&
            buf[8] == 'a' && buf[9] == 't' && buf[10] == 'e' && buf[11] == '\0') {
            return 0;
        }

        if (is_blocked(buf)) {
            return -ENOENT; 
        }

        struct dentry *parent = BPF_CORE_READ(dentry, d_parent);
        /* Stop if we reached the root or a null parent */
        if (!parent || parent == dentry) break; 
        dentry = parent;
    }
    
    return 0;
}