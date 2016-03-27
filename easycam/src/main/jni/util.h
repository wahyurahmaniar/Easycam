#ifndef __UTIL__H__
#define __UTIL__H__

#include <jni.h>
#include <cstdio>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <linux/videodev2.h>
#include <android/log.h>

#define LOG_TAG "NativeEasyCaptureJNI"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO,LOG_TAG,__VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR,LOG_TAG,__VA_ARGS__)

#define CLEAR(x) memset(&(x), 0, sizeof(x))

#define ERROR_LOCAL -1
#define SUCCESS_LOCAL 0

using namespace std;

// Simple buffer that stores a reference to an array and the length of the array
typedef struct {
    void* start;
    size_t length;
} CaptureBuffer;

enum Deinterlace {NONE, DISCARD, BOB, BLEND};

typedef struct {
	char* driver;
	char* location;  		// location of the device file (/dev/videoX)
	int frameWidth;
	int frameHeight;
	int numBuffers;			// number of buffers to allocate

	// The input selection for devices that have multiple inputs.  Set to -1 for devices that have only one input.
	int input;
	Deinterlace deintMethod;

	// V4L2 specific definitions
	v4l2_std_id videoStandard;
	__u32 pixelFormat;
	v4l2_field field;
} DeviceSettings;

int errnoexit(const char *s);

/* Private: Repeat an ioctl call until it completes and is not interrupted by a
 * a signal.
 *
 * The ioctl may still succeed or fail, so do check the return status.
 *
 * fd - the file descriptor for the ioctl.
 * request - the type of IOCTL to request.
 * arg - the target argument for the ioctl.
 *
 * Returns the status of the ioctl when it completes.
 */
int xioctl(int fd, int request, void *arg);

#endif // __UTIL__H__
