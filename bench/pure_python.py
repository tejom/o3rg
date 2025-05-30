import os
import re
from dataclasses import dataclass
from typing import List, Tuple

@dataclass
class SearchMatch:
    line: int
    match_result: str

def search_file(file_name: str, search_regex: str) -> List[SearchMatch]:
    matches = []
    pattern = re.compile(search_regex)
    
    with open(file_name, 'r') as f:
        for i, line in enumerate(f, 1):
            match = pattern.search(line)
            if match:
                matches.append(SearchMatch(
                    line=i,
                    match_result=match.group(0)
                ))
    return matches

def search_dir(dir_path: str, search_regex: str, hidden: bool = False) -> List[Tuple[SearchMatch, str]]:
    matches = []
    pattern = re.compile(search_regex)
    
    for root, _, files in os.walk(dir_path):
        for file in files:
            # Skip hidden files if hidden=False
            if not hidden and file.startswith('.'):
                continue
                
            file_path = os.path.join(root, file)
            try:
                with open(file_path, 'r') as f:
                    for i, line in enumerate(f, 1):
                        match = pattern.search(line)
                        if match:
                            matches.append((
                                SearchMatch(
                                    line=i,
                                    match_result=match.group(0)
                                ),
                                file_path
                            ))
            except (IOError, UnicodeDecodeError):
                # Skip files that can't be read or aren't text
                continue
                
    return matches 
