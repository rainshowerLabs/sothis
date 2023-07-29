import sys
import re

def hex_to_decimal(match):
    hex_str = match.group(0)
    decimal_num = int(hex_str, 16)
    return str(decimal_num)

def convert_hex_to_decimal_in_file(file_path):
    try:
        with open(file_path, 'r') as file:
            content = file.read()

        # Use regular expression to find all hexadecimal numbers in the content
        pattern = r'0x[0-9A-Fa-f]+'
        converted_content = re.sub(pattern, hex_to_decimal, content)

        with open(file_path, 'w') as file:
            file.write(converted_content)

        print(f"Conversion successful. Hex numbers in '{file_path}' converted to decimal.")
    except FileNotFoundError:
        print(f"Error: File '{file_path}' not found.")
    except Exception as e:
        print(f"Error occurred: {e}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <file_path>")
    else:
        file_path = sys.argv[1]
        convert_hex_to_decimal_in_file(file_path)
