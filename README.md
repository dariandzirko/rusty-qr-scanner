<div align="center">

# Title
---
</div>

The goal of this repo was to build a QR code scanner and test the functionality I wrote in my other image processing library <https://github.com/dariandzirko/oxidized-image-processing>.

## Demo
---

I will talk to how both projects are working in tandem as I eventually ended up using this repo as the binary for running the code of my wrapper repo.

### Functionality Goals
---

- [x] Magnitude of Gradient 
- [X] Otsu
- [X] Non-maxima surpression for finding important lines
- [ ] Canny Edge Detector
- [ ] Box Detector
- [ ] Image to QR code
- [ ] QR Decoder

## Proof of Functionality
---

Starting with this ![image](https://github.com/dariandzirko/rusty-qr-scanner/src/images/Plane.jpg)

I was able to make the following images.

Starting with magnitude information of the gradient image of that initial plane, which looks like ![this](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/demo/mag_gradient_image.png)

In another attempt to help find QR codes which should be in the forefront was using Otsu ![this](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/demo/otsu_img.png)

Lastly using the gradient image information, I was able to work through perpendicular line estimates for a lot of the content in the image, and after a double thresholding to find the "strongest" lines I was left ![this](https://github.com/dariandzirko/rusty-qr-scanner/blob/main/demo/double_thresh_image.png)