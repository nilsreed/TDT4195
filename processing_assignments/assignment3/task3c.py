import utils
import skimage
import skimage.morphology
import numpy as np


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
    boundary = im
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
