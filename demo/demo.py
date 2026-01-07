#!/usr/bin/env python3
"""
Simple demo of the dangerzone-rs Python module.

This demonstrates the simplest interface for converting a document to a safe PDF.
"""

import sys
from pathlib import Path

import dangerzone_rs


def main():
    # Example: convert a document to PDF
    input_file = "test_docs/inputs/sample-docx.docx"
    output_file = "safe_output.pdf"
    use_ocr = False  # Set to True to add text layer via OCR

    if not Path(input_file).exists():
        print(f"Error: Input file '{input_file}' not found.")
        print("Please provide a valid document to convert.")
        sys.exit(1)

    try:
        print(f"Converting '{input_file}' to '{output_file}'...")
        print(f"OCR enabled: {use_ocr}")

        # Call the simplest conversion function
        dangerzone_rs.convert_document_py(input_file, output_file, use_ocr)

        print("✓ Conversion complete!")
        print(f"  Output: {output_file}")

    except Exception as e:
        print(f"✗ Conversion failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
