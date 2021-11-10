import matplotlib.pyplot as plt
import numpy as np
import skimage
import utils


def convolve_im(im: np.array,
                kernel: np.array,
                verbose=True):
    """ Convolves the image (im) with the spatial kernel (kernel),
        and returns the resulting image.

        "verbose" can be used for turning on/off visualization
        convolution

        Note: kernel can be of different shape than im.

    Args:
        im: np.array of shape [H, W]
        kernel: np.array of shape [K, K] 
        verbose: bool
    Returns:
        im: np.array of shape [H, W]
    """
    # START YOUR CODE HERE ### (You can change anything inside this block)
    kernel_fft = np.fft.fft2(kernel, [2*im.shape[0], 2*im.shape[1]])
    im_fft = np.fft.fft2(im, [2*im.shape[0], 2*im.shape[1]])
    filtered_fft = np.multiply(kernel_fft,im_fft)
    conv_result = np.fft.ifft2(filtered_fft, [2*im.shape[0], 2*im.shape[1]])
    conv_result = conv_result[0:im.shape[0], 0:im.shape[1]]

    if verbose:
        # Use plt.subplot to place two or more images beside eachother
        plt.figure(figsize=(20, 4))
        # plt.subplot(num_rows, num_cols, position (1-indexed))
        plt.subplot(1, 5, 1)
        plt.imshow(im, cmap="gray")
        plt.subplot(1, 5, 2)
        # Visualize FFT
        plt.imshow(np.log(np.abs(np.fft.fftshift(im_fft)) + 1), cmap='gray')
        plt.subplot(1, 5, 3)
        # Visualize FFT kernel
        plt.imshow(np.log(np.abs(np.fft.fftshift(kernel_fft)) + 1), cmap='gray')
        plt.subplot(1, 5, 4)
        # Visualize filtered FFT image
        plt.imshow(np.log(np.abs(np.fft.fftshift(filtered_fft)) + 1), cmap='gray')
        plt.subplot(1, 5, 5)
        # Visualize filtered spatial image
        plt.imshow(np.real(conv_result), cmap="gray")

    ### END YOUR CODE HERE ###
    return conv_result


if __name__ == "__main__":
    verbose = True  # change if you want

    # Changing this code should not be needed
    im = skimage.data.camera()
    im = utils.uint8_to_float(im)

    # DO NOT CHANGE
    gaussian_kernel = np.array([
        [1, 4, 6, 4, 1],
        [4, 16, 24, 16, 4],
        [6, 24, 36, 24, 6],
        [4, 16, 24, 16, 4],
        [1, 4, 6, 4, 1],
    ]) / 256
    image_gaussian = convolve_im(im, gaussian_kernel, verbose)

    # DO NOT CHANGE
    sobel_horizontal = np.array([
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1]
    ])
    image_sobelx = convolve_im(im, sobel_horizontal, verbose)

    if verbose:
        plt.show()

    utils.save_im("camera_gaussian.png", image_gaussian)
    utils.save_im("camera_sobelx.png", image_sobelx)
