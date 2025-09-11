#ifndef CRUX_CORE_H
#define CRUX_CORE_H

#include <stdint.h>  
#include <stdbool.h> 
#include <stdlib.h>  
#include <stddef.h>  
#include <stdio.h>  

#ifdef __cplusplus
extern "C" {
#endif

char* collect_unique_processes_list(void);

/// Frees a C string 
void free_cstring(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // CRUX_CORE_H
