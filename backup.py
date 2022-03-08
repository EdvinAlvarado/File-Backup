import os
import shutil
import time
import re


def backup(backup_count_limit: int, backup_path: str, file_path: str) -> None:

    # Setup
    file = os.path.basename(file_path)
    filename, ext = os.path.splitext(file)

    # Copy file with timestamp
    now = time.time()
    file_timestamped = f"{filename}-{str(int(now))}{ext}"
    shutil.copy(file_path, backup_path)
    os.rename(os.path.join(backup_path, file), os.path.join(backup_path, file_timestamped))

    # Remove old backups
    regex_str = r"{}-\d*{}".format(filename, ext)
    backups = [name for name in os.listdir(backup_path) if re.search(regex_str, name)]

    for i in range(len(backup_path) - backup_count_limit):
        os.remove(os.path.join(backup_path, backups[i]))
