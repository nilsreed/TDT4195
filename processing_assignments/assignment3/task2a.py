import numpy as np
import skimage
import utils
import pathlib


def otsu_thresholding(im: np.ndarray) -> int:
    """
        Otsu's thresholding algorithm that segments an image into 1 or 0 (True or False)
        The function takes in a grayscale image and outputs a boolean image

        args:
            im: np.ndarray of shape (H, W) in the range [0, 255] (dtype=np.uint8)
        return:
            (int) the computed thresholding value
    """
    assert im.dtype == np.uint8
    # START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    # Compute normalized histogram
    
    # Find the histogram
    histogram = [0]*256
    
    for i in range(im.shape[0]):
        for j in range(im.shape[1]):
            histogram[im[i, j]] += 1

    # Normalize histogram
    normalized_histogram = [0]*256
    
    zeros = 0

    for i in range(256):
        normalized_histogram[i] = histogram[i]/(im.shape[0]*im.shape[1])
        if normalized_histogram[i] == 0:
            zeros+=1
    print(zeros)
    print(normalized_histogram)
    print(sum(normalized_histogram))
        
    # Find cumulative sums
    cum_sums = [0]*256
    cum_sums[0] = normalized_histogram[0]
    for i in range(1, 256):
        cum_sums[i] = cum_sums[i-1] + normalized_histogram[i]
    
    # Find cumulative mean
    cum_means = [0]*256
    # cum_means[0] = 0 #Not necessary
    for i in range(1, 256):
        cum_means[i] = cum_means[i-1] + i*normalized_histogram[i]
    
    # Find global mean
    global_mean = cum_means[-1]

    # Find between-class variance
    sigma_B = [0]*256
    for i in range(256):
        sigma_B[i] = ((global_mean*cum_sums[i] - cum_means[i])**2)/(cum_sums[i]*(1 - cum_sums[i]))
    

    max_sigma = max(sigma_B)

    if sigma_B.count(max_sigma) == 1:
        threshold = sigma_B.index(max_sigma)
    else:
        indices = [i for i, x in enumerate(sigma_B) if x == max_sigma]
        threshold = round(indices.sum()/len(indices))
    return threshold
    ### END YOUR CODE HERE ###


if __name__ == "__main__":
    # DO NOT CHANGE
    impaths_to_segment = [
        pathlib.Path("thumbprint.png"),
        pathlib.Path("polymercell.png")
    ]
    for impath in impaths_to_segment:
        im = utils.read_image(impath)
        threshold = otsu_thresholding(im)
        print("Found optimal threshold:", threshold)

        # Segment the image by threshold
        segmented_image = (im >= threshold)
        assert im.shape == segmented_image.shape, "Expected image shape ({}) to be same as thresholded image shape ({})".format(
            im.shape, segmented_image.shape)
        assert segmented_image.dtype == np.bool, "Expected thresholded image dtype to be np.bool. Was: {}".format(
            segmented_image.dtype)

        segmented_image = utils.to_uint8(segmented_image)

        save_path = "{}-segmented.png".format(impath.stem)
        utils.save_im(save_path, segmented_image)
