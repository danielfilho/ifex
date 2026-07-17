#!/usr/bin/env python3
"""
Creates sample JPEG images with identical EXIF creation dates for testing the date adjustment feature.
"""

from PIL import Image
from PIL.ExifTags import TAGS
import piexif
from datetime import datetime
import os

def create_sample_jpeg_with_date(filename, date_str, width=100, height=100):
    """Create a simple JPEG with EXIF date information."""
    
    # Create a simple colored image
    image = Image.new('RGB', (width, height), color='red')
    
    # Create EXIF data with the specified date
    exif_dict = {
        "0th": {
            piexif.ImageIFD.Make: "Test Camera",
            piexif.ImageIFD.Model: "Test Model",
            piexif.ImageIFD.DateTime: date_str,
            piexif.ImageIFD.Software: "Test Script",
        },
        "Exif": {
            piexif.ExifIFD.DateTimeOriginal: date_str,
            piexif.ExifIFD.DateTimeDigitized: date_str,
        },
    }
    
    exif_bytes = piexif.dump(exif_dict)
    
    # Save the image with EXIF data
    image.save(filename, "JPEG", exif=exif_bytes)
    print(f"Created {filename} with date {date_str}")

def main():
    """Create test images with identical dates."""
    
    # Ensure the examples directory exists
    os.makedirs("examples/test_photos", exist_ok=True)
    
    # Create images with the same creation date
    test_date = "2024:01:15 14:30:00"
    
    create_sample_jpeg_with_date("examples/test_photos/IMG_001.jpg", test_date)
    create_sample_jpeg_with_date("examples/test_photos/IMG_002.jpg", test_date)
    create_sample_jpeg_with_date("examples/test_photos/IMG_003.jpg", test_date)
    create_sample_jpeg_with_date("examples/test_photos/IMG_004.jpg", test_date)
    
    print("\nCreated 4 test images with identical creation dates.")
    print("These can be used to test the automatic date adjustment feature.")
    print("Run: cargo run -- and select 'Apply EXIF data to images' to test the feature.")

if __name__ == "__main__":
    main()