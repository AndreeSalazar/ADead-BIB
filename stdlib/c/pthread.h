/*
 * ADead-BIB Standard Library
 * pthread.h - POSIX Threads
 * 
 * Based on: POSIX.1-2017
 */

#ifndef _ADEAD_PTHREAD_H
#define _ADEAD_PTHREAD_H

#include "sys/types.h"
#include "time.h"

/* Thread types */
typedef unsigned long pthread_t;
typedef unsigned int pthread_key_t;
typedef int pthread_once_t;

/* Mutex types */
typedef struct {
    int __lock;
    unsigned int __count;
    int __owner;
    int __kind;
    unsigned int __nusers;
    int __spins;
} pthread_mutex_t;

typedef struct {
    int __kind;
} pthread_mutexattr_t;

/* Condition variable */
typedef struct {
    int __lock;
    unsigned int __futex;
    unsigned long long __total_seq;
    unsigned long long __wakeup_seq;
    unsigned long long __woken_seq;
    void* __mutex;
    unsigned int __nwaiters;
    unsigned int __broadcast_seq;
} pthread_cond_t;

typedef struct {
    int __clock;
} pthread_condattr_t;

/* Read-write lock */
typedef struct {
    int __lock;
    unsigned int __nr_readers;
    unsigned int __readers_wakeup;
    unsigned int __writer_wakeup;
    unsigned int __nr_readers_queued;
    unsigned int __nr_writers_queued;
    int __writer;
    int __shared;
    unsigned long __pad1;
    unsigned long __pad2;
    unsigned int __flags;
} pthread_rwlock_t;

typedef struct {
    int __pshared;
} pthread_rwlockattr_t;

/* Barrier */
typedef struct {
    unsigned int __count;
    unsigned int __current;
    int __lock;
    unsigned int __futex;
} pthread_barrier_t;

typedef struct {
    int __pshared;
} pthread_barrierattr_t;

/* Thread attributes */
typedef struct {
    int __detachstate;
    int __schedpolicy;
    struct sched_param __schedparam;
    int __inheritsched;
    int __scope;
    size_t __guardsize;
    int __stackaddr_set;
    void* __stackaddr;
    size_t __stacksize;
} pthread_attr_t;

/* Initializers */
#define PTHREAD_MUTEX_INITIALIZER { 0, 0, 0, 0, 0, 0 }
#define PTHREAD_COND_INITIALIZER { 0, 0, 0, 0, 0, 0, 0, 0 }
#define PTHREAD_RWLOCK_INITIALIZER { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 }
#define PTHREAD_ONCE_INIT 0

/* Detach state */
#define PTHREAD_CREATE_JOINABLE 0
#define PTHREAD_CREATE_DETACHED 1

/* Mutex types */
#define PTHREAD_MUTEX_NORMAL     0
#define PTHREAD_MUTEX_RECURSIVE  1
#define PTHREAD_MUTEX_ERRORCHECK 2
#define PTHREAD_MUTEX_DEFAULT    PTHREAD_MUTEX_NORMAL

/* Process shared */
#define PTHREAD_PROCESS_PRIVATE 0
#define PTHREAD_PROCESS_SHARED  1

/* Scope */
#define PTHREAD_SCOPE_SYSTEM  0
#define PTHREAD_SCOPE_PROCESS 1

/* Cancel */
#define PTHREAD_CANCEL_ENABLE  0
#define PTHREAD_CANCEL_DISABLE 1
#define PTHREAD_CANCEL_DEFERRED     0
#define PTHREAD_CANCEL_ASYNCHRONOUS 1
#define PTHREAD_CANCELED ((void*)-1)

/* Thread functions */
int pthread_create(pthread_t* thread, const pthread_attr_t* attr,
                   void* (*start_routine)(void*), void* arg);
void pthread_exit(void* retval);
int pthread_join(pthread_t thread, void** retval);
int pthread_detach(pthread_t thread);
pthread_t pthread_self(void);
int pthread_equal(pthread_t t1, pthread_t t2);

/* Thread attributes */
int pthread_attr_init(pthread_attr_t* attr);
int pthread_attr_destroy(pthread_attr_t* attr);
int pthread_attr_setdetachstate(pthread_attr_t* attr, int detachstate);
int pthread_attr_getdetachstate(const pthread_attr_t* attr, int* detachstate);
int pthread_attr_setstacksize(pthread_attr_t* attr, size_t stacksize);
int pthread_attr_getstacksize(const pthread_attr_t* attr, size_t* stacksize);

/* Mutex functions */
int pthread_mutex_init(pthread_mutex_t* mutex, const pthread_mutexattr_t* attr);
int pthread_mutex_destroy(pthread_mutex_t* mutex);
int pthread_mutex_lock(pthread_mutex_t* mutex);
int pthread_mutex_trylock(pthread_mutex_t* mutex);
int pthread_mutex_unlock(pthread_mutex_t* mutex);
int pthread_mutex_timedlock(pthread_mutex_t* mutex, const struct timespec* abstime);

/* Mutex attributes */
int pthread_mutexattr_init(pthread_mutexattr_t* attr);
int pthread_mutexattr_destroy(pthread_mutexattr_t* attr);
int pthread_mutexattr_settype(pthread_mutexattr_t* attr, int type);
int pthread_mutexattr_gettype(const pthread_mutexattr_t* attr, int* type);

/* Condition variable functions */
int pthread_cond_init(pthread_cond_t* cond, const pthread_condattr_t* attr);
int pthread_cond_destroy(pthread_cond_t* cond);
int pthread_cond_wait(pthread_cond_t* cond, pthread_mutex_t* mutex);
int pthread_cond_timedwait(pthread_cond_t* cond, pthread_mutex_t* mutex,
                           const struct timespec* abstime);
int pthread_cond_signal(pthread_cond_t* cond);
int pthread_cond_broadcast(pthread_cond_t* cond);

/* Read-write lock functions */
int pthread_rwlock_init(pthread_rwlock_t* rwlock, const pthread_rwlockattr_t* attr);
int pthread_rwlock_destroy(pthread_rwlock_t* rwlock);
int pthread_rwlock_rdlock(pthread_rwlock_t* rwlock);
int pthread_rwlock_tryrdlock(pthread_rwlock_t* rwlock);
int pthread_rwlock_wrlock(pthread_rwlock_t* rwlock);
int pthread_rwlock_trywrlock(pthread_rwlock_t* rwlock);
int pthread_rwlock_unlock(pthread_rwlock_t* rwlock);

/* Barrier functions */
int pthread_barrier_init(pthread_barrier_t* barrier, const pthread_barrierattr_t* attr,
                         unsigned int count);
int pthread_barrier_destroy(pthread_barrier_t* barrier);
int pthread_barrier_wait(pthread_barrier_t* barrier);
#define PTHREAD_BARRIER_SERIAL_THREAD (-1)

/* Thread-specific data */
int pthread_key_create(pthread_key_t* key, void (*destructor)(void*));
int pthread_key_delete(pthread_key_t key);
void* pthread_getspecific(pthread_key_t key);
int pthread_setspecific(pthread_key_t key, const void* value);

/* Once */
int pthread_once(pthread_once_t* once_control, void (*init_routine)(void));

/* Cancellation */
int pthread_cancel(pthread_t thread);
int pthread_setcancelstate(int state, int* oldstate);
int pthread_setcanceltype(int type, int* oldtype);
void pthread_testcancel(void);

#endif /* _ADEAD_PTHREAD_H */
