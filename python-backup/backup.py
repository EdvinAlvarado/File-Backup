import os
import shutil
import time
import re


def backup(backup_count: int, backup_dir: str, source_file: str) -> None:

    # Setup
    database = os.path.basename(source_file)
    filename, ext = os.path.splitext(database)

    # Copy file with timestamp
    now = time.time()
    timestamp_name = f"{filename}-{str(int(now))}{ext}"
    shutil.copy(source_file, backup_dir)
    os.rename(os.path.join(backup_dir, database), os.path.join(backup_dir, timestamp_name))

    # Remove old backups
    regex_str = r"{}-\d*{}".format(filename, ext)
    backup_files = [name for name in os.listdir(backup_dir) if re.search(regex_str, name)] 

    for i in range(len(backup_files) - backup_count):
        os.remove(os.path.join(backup_dir, backup_files[i]))
