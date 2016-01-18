
#pragma version(1)
#pragma rs java_package_name(com.arksine.easycam)
#pragma rs_fp_relaxed

rs_allocation inAllocation;
rs_allocation outAllocation;

int32_t firstElement;   //  The first Element to be processed. For interleaved frames the first element 0 for
						//  Odd frames, (frame Width / 2) for even frames in YUY2 or other 16-bit
						//  formats, and frameWidth for RGBA and other 32-bit formats.  For sequential frames
						//  0 is the first element of the first half, and (frame width * frameheight) / 2, which is halfway,
						//  for the second half

// Converts whole frame from YUYV to RGBA
void __attribute__((kernel)) convertFrameFromYUYV(int32_t in, uint32_t x)
{
	uchar4 inElement;
    uchar4 outElement;
    uchar yValue;

    inElement = rsGetElementAt_uchar4(inAllocation, (x / 2));

	if ((x & (int32_t)0x0001) == 0)
	{
		//First pixel in a pair of YUV pixels
		yValue = inElement.x;
	}
	else
	{
		yValue = inElement.z;
	}

	outElement = rsYuvToRGBA_uchar4(yValue, inElement.y, inElement.w);

	rsSetElementAt_uchar4(outAllocation, outElement, x);

}

// Converts the even or odd fields in Interlaced Frame or sequential Frames.  The first element variable tells us which
// field to process, and the x values in the pixel alloc will determine if the frame is interlaced or sequential.
void __attribute__((kernel)) convertFieldFromYUYV(int32_t xIn, uint32_t x)
{
    uchar4 inElement;
    uchar4 outElement;
    uchar yValue;

	// The first element is an offset we add to the index received from the pixel alloc
    int32_t inputIndex = xIn + firstElement;

    inElement = rsGetElementAt_uchar4(inAllocation, inputIndex);

	if ((x & (int32_t)0x0001) == 0)
	{
		//First pixel in a pair of YUV pixels
		yValue = inElement.x;
	}
	else
	{
		yValue = inElement.z;
	}

	outElement = rsYuvToRGBA_uchar4(yValue, inElement.y, inElement.w);

	rsSetElementAt_uchar4(outAllocation, outElement, x);

}



// Converts whole frame from UYVY to RGBA
void __attribute__((kernel)) convertFrameFromUYVY(int32_t in, uint32_t x)
{
	uchar4 inElement;
    uchar4 outElement;
    uchar yValue;

    inElement = rsGetElementAt_uchar4(inAllocation, (x / 2));

	if ((x & (int32_t)0x0001) == 0)
	{
		//First pixel in a pair of YUV pixels
		yValue = inElement.y;
	}
	else
	{
		yValue = inElement.w;
	}

	outElement = rsYuvToRGBA_uchar4(yValue, inElement.x, inElement.z);

	rsSetElementAt_uchar4(outAllocation, outElement, x);

}

// Converts the even or odd fields in Interlaced Frame or sequential Frames.  The first element variable tells us which
// field to process, and the x values in the pixel alloc will determine if the frame is interlaced or sequential.
void __attribute__((kernel)) convertFieldromUYVY(int32_t xIn, uint32_t x)
{
    uchar4 inElement;
    uchar4 outElement;
    uchar yValue;

    int32_t inputIndex = xIn + firstElement;

    inElement = rsGetElementAt_uchar4(inAllocation, inputIndex);

    if ((x & (int32_t)0x0001) == 0)
    {
        //First pixel in a pair of YUV pixels
        yValue = inElement.y;
    }
    else
    {
        yValue = inElement.w;
    }

    outElement = rsYuvToRGBA_uchar4(yValue, inElement.x, inElement.z);

   	rsSetElementAt_uchar4(outAllocation, outElement, x);

}