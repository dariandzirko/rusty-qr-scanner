<div align="center">

# rusty-qr-scanner
---
</div>

The goal of this repo is to build a QR code scanner and test the functionality I wrote in my other image processing library <https://github.com/dariandzirko/oxidized-image-processing>.

## Functionality Goals
---

- [x] Magnitude of Gradient 
- [X] Otsu
- [X] Non-maxima surpression with Double Thresholding
- [ ] Canny Edge Detector
- [ ] Box Detector
- [ ] Image to QR code
- [ ] QR Decoder

## What's Working and How
---

<div align="center">

#### Base image: 
![base](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/src/images/Plane.jpg)

</div>

Starting with the base image I made the following pretty cool images.

First convolving (my own custom convolution from my own ![oxidized image processing library](https://github.com/dariandzirko/oxidized-image-processing)) the original image and a Sobel derivative filter in each direction. With the X and Y derivatives it is just "square square, square root" for magnitude and atan2(Dy/Dx) for angle. Now I have the gradient information for each pixel of the original image, and by displaying just the magnitude information I get 

<div align="center">

![this](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/demo/mag_gradient_image.png)

</div>

With the angle information and a nice switch statement I can determine the direction of edge lines. The angle content tells us the direction that pixels are changing. Meaning if the angle is pi/2 then the pixels are changing in the y/vertical direction, meaning it is a x/horizontal edge. I classified 4 edges, as -pi/2 and pi/2 would still be a horizontal edge. Now I have a bunch of ideas of lines, that can even be directly next to eachother if there is a multi-pixel edge. Now time for non-maxima suppression implemented where I picked the largest gradient magnitude of adjacent pixels in the value based on edge type. For example a horizontal edge, check the pixel above and below it (the direction of pixel value change) to make sure it is the strongest of edge out of nearby potential horizontal edges. Now with all the local maxima lines left, I use a double threshold on the remaining gradient values, highlighting the most prominent edges while keeping the lower value edges and removing the least prominent edges. Resulting in this 

<div align="center">

![image](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/demo/double_thresh_image.png)

</div>

The Otsu functionality just seemed useful and fun to try interweaving it, which may help find the more prominent features. More specifically when it can to QR codes, find the outline of the QR code. Based on the original image, applying Otsu leads to 

<div align="center">

![this](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/demo/otsu_img.png)

</div>
