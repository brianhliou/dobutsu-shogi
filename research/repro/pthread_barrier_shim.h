/*
 * pthread_barrier_shim.h — portable pthread barrier for platforms (macOS)
 * whose libc omits the optional POSIX barrier API. No-op where the platform
 * already provides barriers.
 *
 * Used to build clausecker/dobutsu on macOS — copy into the clone and
 * #include it near the top of tbgenerate.c. See ../reproduction.md.
 */
#ifndef PTHREAD_BARRIER_SHIM_H
#define PTHREAD_BARRIER_SHIM_H

#include <pthread.h>
#include <errno.h>

#ifndef PTHREAD_BARRIER_SERIAL_THREAD

typedef int pthread_barrierattr_t;

typedef struct {
	pthread_mutex_t mutex;
	pthread_cond_t  cond;
	unsigned int    count;    /* threads required to trip the barrier */
	unsigned int    waiting;  /* threads currently waiting            */
	unsigned int    phase;    /* generation counter                   */
} pthread_barrier_t;

#define PTHREAD_BARRIER_SERIAL_THREAD 1

static inline int
pthread_barrier_init(pthread_barrier_t *b,
    const pthread_barrierattr_t *attr, unsigned int count)
{
	(void)attr;
	if (count == 0) { errno = EINVAL; return EINVAL; }
	if (pthread_mutex_init(&b->mutex, NULL) != 0) return errno;
	if (pthread_cond_init(&b->cond, NULL) != 0) {
		pthread_mutex_destroy(&b->mutex);
		return errno;
	}
	b->count = count;
	b->waiting = 0;
	b->phase = 0;
	return 0;
}

static inline int
pthread_barrier_destroy(pthread_barrier_t *b)
{
	pthread_cond_destroy(&b->cond);
	pthread_mutex_destroy(&b->mutex);
	return 0;
}

static inline int
pthread_barrier_wait(pthread_barrier_t *b)
{
	unsigned int phase;

	pthread_mutex_lock(&b->mutex);
	phase = b->phase;

	if (++b->waiting == b->count) {
		b->phase++;
		b->waiting = 0;
		pthread_cond_broadcast(&b->cond);
		pthread_mutex_unlock(&b->mutex);
		return PTHREAD_BARRIER_SERIAL_THREAD;
	}

	while (phase == b->phase)
		pthread_cond_wait(&b->cond, &b->mutex);

	pthread_mutex_unlock(&b->mutex);
	return 0;
}

#endif /* PTHREAD_BARRIER_SERIAL_THREAD */
#endif /* PTHREAD_BARRIER_SHIM_H */
