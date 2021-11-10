import skimage
import skimage.io
import skimage.transform
import os
import numpy as np
import utils
import matplotlib.pyplot as plt


# Copied from 4a
def convolve_im(im: np.array,
                fft_kernel: np.array):
    """ Convolves the image (im) with the frequency kernel (fft_kernel),
        and returns the resulting image.

    Args:
        im: np.array of shape [H, W]
        fft_kernel: np.array of shape [H, W] 
    Returns:
        im: np.array of shape [H, W]
    """
    # START YOUR CODE HERE ### (You can change anything inside this block)
    im_fft = np.fft.fft2(im)#, [2*im.shape[0], 2*im.shape[1]])
    filtered_fft = np.multiply(im_fft, fft_kernel)
    conv_result = np.fft.ifft2(filtered_fft)#,[2*im.shape[0], 2*im.shape[1]])

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
    plt.imshow(np.log(np.abs(np.fft.fftshift(fft_kernel)) + 1), cmap='gray')
    plt.subplot(1, 5, 4)
    # Visualize filtered FFT image
    plt.imshow(np.log(np.abs(np.fft.fftshift(filtered_fft)) + 1), cmap='gray')
    plt.subplot(1, 5, 5)
    # Visualize filtered spatial image
    plt.imshow(np.real(conv_result), cmap="gray")

    ### END YOUR CODE HERE ###
    return conv_result

if __name__ == "__main__":
    # DO NOT CHANGE
    impath = os.path.join("images", "noisy_moon.png")
    im = utils.read_im(impath)

    # START YOUR CODE HERE ### (You can change anything inside this block)
    fft_filter = np.ones([im.shape[0], im.shape[1]])

    fft_filter[267:272, :] = 0
    fft_filter[267:272, 205:259] = 1
        

    im_filtered = convolve_im(im, np.fft.fftshift(fft_filter))

    plt.show()

    ### END YOUR CODE HERE ###
    utils.save_im("moon_filtered.png", utils.normalize(im_filtered))
