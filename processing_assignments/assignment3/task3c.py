import utils
import skimage
import skimage.morphology
import numpy as np

def pixel_bool_subtract(A: bool, B: bool) -> bool:
    if (not B):
        return A
    return False

def image_bool_subtract(im: np.ndarray, sub_im: np.ndarray) -> np.ndarray:
    assert im.dtype == np.bool
    assert sub_im.dtype == np.bool
    assert im.shape == sub_im.shape

    for x in range(im.shape[0]):
        for y in range(im.shape[1]):
            im[x, y] = pixel_bool_subtract(im[x, y], sub_im[x, y])
    
    return im

def extract_boundary(im: np.ndarray) -> np.ndarray:
    """
        A function that extracts the inner boundary from a boolean image.

        args:
            im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
        return:
            (np.ndarray) of shape (H, W). dtype=np.bool
    """
    # START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    structuring_element = np.array([
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1]
    ], dtype=bool)

    eroded = skimage.morphology.binary_erosion(im, structuring_element)

    boundary = image_bool_subtract(im, eroded)
    
    return boundary
    ### END YOUR CODE HERE ###


if __name__ == "__main__":
    im = utils.read_image("lincoln.png")
    binary_image = (im != 0)
    boundary = extract_boundary(binary_image)

    assert im.shape == boundary.shape, "Expected image shape ({}) to be same as resulting image shape ({})".format(
        im.shape, boundary.shape)
    assert boundary.dtype == np.bool, "Expected resulting image dtype to be np.bool. Was: {}".format(
        boundary.dtype)

    boundary = utils.to_uint8(boundary)
    utils.save_im("lincoln-boundary.png", boundary)
