import os

import requests
import platformdirs

# input download wrapper, takes these env variables for config:
# ADVENT_AUTH_SESSION_ID
# ADVENT_GET_REAL_INPUT

advent_headers = {
    'Cookie': 'session='+os.environ['ADVENT_AUTH_SESSION_ID'],
    'User-Agent': f'python-requests/{requests.__version__} (custom tooling by https://github.com/mschuett/)'
}

def is_example() -> bool:
    return not bool(os.environ.get('ADVENT_GET_REAL_INPUT', False))

def get_input_data(year=2024, day=1, example_data="") -> str:
    """provide example data or real input data for day of year, depending on ADVENT_GET_REAL_INPUT.
    real input data is cached. download needs a session cookie in ADVENT_AUTH_SESSION_ID"""
    if not os.environ.get('ADVENT_GET_REAL_INPUT', False):
        return example_data

    cache_base = platformdirs.user_cache_path(appname="adventofcode", appauthor=False, ensure_exists=True)
    cache_dir = os.path.join(cache_base, str(year))
    cache_file = os.path.join(cache_dir, f"{day}.txt")
    try:
        with open(cache_file, 'r') as f:
            return f.read()
    except OSError:
        pass
    os.makedirs(cache_dir, exist_ok=True)
    input_data = requests.get(f'https://adventofcode.com/{year}/day/{day}/input', headers=advent_headers).text
    with open(cache_file, 'w') as f:
        f.write(input_data)
    return input_data
