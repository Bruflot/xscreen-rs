def rgba_to_premul(red, green, blue, alpha):
    color =  (red << 16) + (green << 8) + blue
    return (color & 0x00FFFFFF) | (alpha << 24)

print(rgba_to_premul(0, 0, 0, 130))