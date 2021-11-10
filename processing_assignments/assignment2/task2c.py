

import matplotlib.pyplot as plt
from PIL import Image
import torchvision
import torch
import numpy as np


image = Image.open("images/zebra.jpg")
plt.imshow(image)
print("Image shape:", image.size)


# In this example we will use a pre-trained ResNet50 network.
# ResNet-50 is a fully-convolutional neural network that excels at image classification.
model = torchvision.models.resnet50(pretrained=True)
print(model)


# In this task we are interested in visualizing the first convolutional layer. This can be retrieved by the following code block:
first_conv_layer = model.conv1
print("First conv layer weight shape:", first_conv_layer.weight.shape)
print("First conv layer:", first_conv_layer)
# Observe that it has 64 filters/kernels in the layer. Each kernel is a $7 \times 7$ filter, that takes an RGB image as input


# We need to resize, and normalize the image with the mean and standard deviation that they used to originally train this network.
image_transform = torchvision.transforms.Compose([
    torchvision.transforms.Resize((224, 224)),
    torchvision.transforms.ToTensor(),
    torchvision.transforms.Normalize(
        [0.485, 0.456, 0.406], [0.229, 0.224, 0.225])
])


# Apply the image transform to the zebra image
image = image_transform(image)[None]
print("Image shape:", image.shape)
# By running the image through the first layer, we get an activation.
# We can retrieve the activation from the first layer by doing a forward pass throught this conv layer.
activation = first_conv_layer(image)
print("Activation shape:", activation.shape)


# Also, we can retrieve the weight from the first convolution layer with the following:
weight = model.conv1.weight.data.cpu()
print("Filter/Weight/kernel size:", weight.shape)


def torch_image_to_numpy(image: torch.Tensor):
    """
        We've created a function `torch_image_to_numpy` to help you out.
        This function transforms an torch tensor with shape (batch size, num channels, height, width) to
        (batch size, height, width, num channels) numpy array
    """
    # Normalize to [0 - 1.0]
    image = image.detach().cpu()  # Transform image to CPU memory (if on GPU VRAM)
    image = image - image.min()
    image = image / image.max()
    image = image.numpy()
    if len(image.shape) == 2:  # Grayscale image, can just return
        return image
    assert image.shape[0] == 3, "Expected color channel to be on first axis. Got: {}".format(
        image.shape)
    image = np.moveaxis(image, 0, 2)
    return image


# START YOUR CODE HERE ### (You can change anything inside this block)
# plt.subplot is a nice function to use for this task!
# Tip: test out for indices = [01,2,3,4,5] to check that your result is correct!
indices = [5, 8, 19, 22, 34]# [0, 1, 2, 3, 4, 5]
num_filters = len(indices)
# %%

plt.figure(figsize=(20, 4))
n = 1
for i in indices:
    plt.subplot(2, num_filters, n)
    # Plot weight here
    plt.imshow(torch_image_to_numpy(weight[i, :, :, :]))
    plt.subplot(2, num_filters, num_filters+n)
    # Plot activation here
    plt.imshow(torch_image_to_numpy(activation[0, i, :, :]), cmap='gray')
    n += 1
plt.savefig("image_processed/2c.png")
plt.show()
### END YOUR CODE HERE ###
# 4b
# weight = list(model.children())[1].weight.cpu().data
# print(type(weight))
# print(weight.shape)

# for i in range(10):
#     im = torch.reshape(weight[i, :], (28, 28))
#     plt.imshow(im)
#     plt.savefig("image_solutions/" + str(i) + "_weight.png")
#     plt.show()
