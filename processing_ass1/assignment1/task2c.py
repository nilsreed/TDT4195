import matplotlib.pyplot as plt
import pathlib
import numpy as np
from utils import read_im, save_im, normalize
output_dir = pathlib.Path("image_solutions")
output_dir.mkdir(exist_ok=True)


im = read_im(pathlib.Path("images", "lake.jpg"))
plt.imshow(im)


def convolve_im(im, kernel,
                ):
    """ A function that convolves im with kernel

    Args:
        im ([type]): [np.array of shape [H, W, 3]]
        kernel ([type]): [np.array of shape [K, K]]

    Returns:
        [type]: [np.array of shape [H, W, 3]. should be same as im]
    """
    assert len(im.shape) == 3

    # Flip kernel, so that we do correlation instead of convolution
    corr_kernel = np.fliplr(kernel.copy())
    corr_kernel = np.flipud(corr_kernel)

    # Layers of zero padding as a function of K.
    # Assuming K is odd, (K-1)/2 layers of zeros is needed
    padding = int((corr_kernel.shape[0] - 1)/2)
    
    
    # Make temporary array to store zero-padded image in while it's being convolved
    # This is done so that we can place the values directly back into im while convolved
    padded = np.zeros((im.shape[0] + 2*padding, im.shape[1] + 2*padding, 3))
    padded[padding:im.shape[0]+padding, padding:im.shape[1]+padding, :] = im

    # Do convolution on each colour channel
    for channel in range(3):
        for x in range(im.shape[0]):
            for y in range(im.shape[1]):
                im[x, y, channel] =  0
                for u in range(-padding, padding + 1):
                    for v in range(-padding, padding + 1):
                        im[x, y, channel] += padded[x + padding + u, y + padding + v, channel]*corr_kernel[u, v]

    return im


if __name__ == "__main__":
    # Define the convolutional kernels
    h_b = 1 / 256 * np.array([
        [1, 4, 6, 4, 1],
        [4, 16, 24, 16, 4],
        [6, 24, 36, 24, 6],
        [4, 16, 24, 16, 4],
        [1, 4, 6, 4, 1]
    ])
    sobel_x = np.array([
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1]
    ])

    # Convolve images
    im_smoothed = convolve_im(im.copy(), h_b)
    save_im(output_dir.joinpath("im_smoothed.jpg"), im_smoothed)
    im_sobel = convolve_im(im, sobel_x)
    save_im(output_dir.joinpath("im_sobel.jpg"), im_sobel)

    # DO NOT CHANGE. Checking that your function returns as expected
    assert isinstance(
        im_smoothed, np.ndarray),         f"Your convolve function has to return a np.array. " + f"Was: {type(im_smoothed)}"
    assert im_smoothed.shape == im.shape,         f"Expected smoothed im ({im_smoothed.shape}" + \
        f"to have same shape as im ({im.shape})"
    assert im_sobel.shape == im.shape,         f"Expected smoothed im ({im_sobel.shape}" + \
        f"to have same shape as im ({im.shape})"
    plt.subplot(1, 2, 1)
    plt.imshow(normalize(im_smoothed))

    plt.subplot(1, 2, 2)
    plt.imshow(normalize(im_sobel))
    plt.show()
