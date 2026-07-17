# IFEX Examples

## Date Adjustment Feature

This directory contains examples and tools for testing the automatic date adjustment feature.

### Feature Description

The date adjustment feature automatically handles photos with identical creation dates:

1. **Automatic Mode**: When all selected photos have the same creation date and time, the tool automatically adjusts them with 1-second increments.
   - First photo keeps the original date
   - Second photo gets original date + 1 second
   - Third photo gets original date + 2 seconds
   - And so on...

2. **Manual Mode**: When photos have different creation dates, the user is prompted (y/N default to No):
   - If Yes: All photos get the same base date (from first photo) with 1-second increments
   - If No: No date adjustment is performed

### Testing the Feature

1. **Create Test Images** (requires Python with Pillow and piexif):
   ```bash
   python3 examples/create_test_images.py
   ```

2. **Run IFEX and Test**:
   ```bash
   cargo run
   ```
   - Select "Apply EXIF data to images"
   - Choose your equipment setup
   - Select the `examples/test_photos` folder
   - Select all the test images
   - Watch the automatic date adjustment in action!

3. **Verify Results**:
   Use any EXIF viewer to check that the photos now have incremental timestamps.

### Manual Testing

You can also test with your own photos that have identical timestamps from lab scans.

### Important Notes

- The feature only works during EXIF application (not during erasure)
- Photos are sorted by filename before date adjustment
- Only JPEG files have full date modification support
- TIFF files will show a note that full support is not yet implemented
- RAW files will create/update XMP sidecar files with date information