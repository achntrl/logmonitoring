import re
import time
import sys
from datetime import datetime, timezone
import pytz


def replace_time(line):
    """Replace time in the """
    current_time = datetime.now(pytz.timezone("Europe/Paris")).strftime('%d/%b/%Y:%H:%M:%S %z')
    current_time = '[' + current_time + ']'
    return re.sub(r'\[.+\]', current_time, line)


def log_writter(interval):
    """Write in a file at a given interval.

    It takes the input from a source file and write to a target file

    Keyword arguments:
    interval -- the duration between each write
    """
    with open('data/clarknet_access_log_truncated.txt', 'r') as source:
        with open('data/server.log', 'a') as target:
            i = 0
            while True:
                line = source.readline()
                target.write(replace_time(line))
                i += 1
                target.flush()
                time.sleep(interval)


if __name__ == '__main__':
    if len(sys.argv) > 1:
        try:
            interval = float(sys.argv[1])
        except ValueError:
            print("Usage: python log_writer.py <interval>")
            print("<interval> is the duration in second between each log")
            exit(-1)
    else:
        interval = 1

    try:
        log_writter(interval)
    except KeyboardInterrupt:
        print('\nExit!')
