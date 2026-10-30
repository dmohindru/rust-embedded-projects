## What next

1. Think my original idea of driver holding the whole framebuffer in memory is not good for embedded targets like Mirobit or any other mcu. It worked for ssd1306 because the buffer size was quite small, a single byte could address almost 8 pixels. So the mental model that I am thinking of is
2. Consider display ram as some kind of external ram, accessable via spi or i2c
3. Display controller commands acts as a way to access/update this ram
4. Let the driver be stateless
5. State to be held by application like a bouncing ball program that holds the state of application. WHich I believe will not take too much of ram
6. Application state is converted to pixel data by embedded graphics library.
7. For this to work efficiently, in one of the other thread I have started discussion of dirty rectangle trick to only send the partial update (think of this as updating only the section of display ram.
8. About the dirty rectangle trick I am thinking of it as an analogy of filesystem that sits between application and block storage device.
9. Applications only talk to this layer. Then this layer in turn talks to display device to send the required data to it.
10. Current approach was taken since it was easy and didn't had to develop this intermediate layer
