from .helper import get_input_data
from typing import List,Set,Dict,Tuple,Union

class FileSnippet(object):
    start: int
    length: int
    file: int
    def __init__(self,start,length,file):
        self.start = start
        self.length = length
        self.file = file
    def __repr__(self):
        return f"({self.start},{self.length},{self.file})"


class DiskFragMap(object):
    fileblocklist: List[FileSnippet]
    def __init__(self,input_text: str) -> None:
        self.fileblocklist = []
        is_free = False
        file = 0
        block = 0
        for char in input_text.strip():
            length = int(char)
            if is_free:
                cur_file = -1
            else:
                cur_file = file
                file += 1
            if length > 0:
                self.fileblocklist.append(FileSnippet(block, length, cur_file))
            block += length
            is_free = not is_free

    def __str__(self):
        outstr = ""
        block = 0
        self.sort()
        for snippet in self.fileblocklist:
            assert snippet.start == block, f"start {snippet.start} != block {block}, in {self.fileblocklist}"
            if snippet.file == -1:
                filename = '.'
            else:
                filename = str(snippet.file)
            outstr += snippet.length * filename
            block += snippet.length
        return outstr #  + "\n" + str(self.fileblocklist)

    def sort(self):
        self.fileblocklist.sort(key=lambda x: x.start)

    def get_first_free_space(self, min_size = 0) -> Union[None,FileSnippet]:
        return next((x for x in self.fileblocklist if x.file == -1 and x.length >= min_size), None)

    def has_free_space(self) -> bool:
        return bool(self.get_first_free_space())

    def get_last_file_space(self) -> FileSnippet:
        snippet = self.fileblocklist[-1]
        if snippet.file == -1:
            # cleanup and remove trailing free space
            self.fileblocklist.remove(snippet)
            del snippet
            return self.get_last_file_space()
        else:
            return snippet

    def defrag_step(self):
        last_file = self.get_last_file_space()
        first_free_space = self.get_first_free_space()
        if not (last_file.file and first_free_space): return

        # move multiple blocks in one step
        move_length = min(first_free_space.length, last_file.length)
        self.move_part(last_file, first_free_space, move_length)

    def move_part(self, src, dst, move_length):
        if move_length == dst.length:
            # take full snippet
            dst.file = src.file
            src.length -= move_length
        else:
            # partial move => split free space
            new_snippet = FileSnippet(dst.start, move_length, src.file)
            self.fileblocklist.insert(self.fileblocklist.index(dst), new_snippet)
            dst.start += move_length
            dst.length -= move_length
            src.length -= move_length
        if src.length == 0:
            self.fileblocklist.remove(src)
            del src
        if dst.length == 0:
            self.fileblocklist.remove(dst)
            del dst

    def checksum(self) -> int:
        block = 0
        acc = 0
        for snippet in self.fileblocklist:
            for i in range(snippet.start, snippet.start + snippet.length):
                if snippet.file != -1:
                    acc += i * snippet.file
            block += snippet.length
        return acc

    def defrag_fullfile(self):
        max_file_id = self.get_last_file_space().file
        for file_id in range(max_file_id, 0, -1):
            self.defrag_fullfile_step(file_id)

    def get_snippet_of_file(self, file_id: int) -> FileSnippet:
        snippets = [x for x in self.fileblocklist if x.file == file_id]
        assert len(snippets) == 1
        return snippets[0]

    def defrag_fullfile_step(self, file_id: int) -> bool:
        snippet = self.get_snippet_of_file(file_id)
        freespace = self.get_first_free_space(snippet.length)
        if not freespace or snippet.start < freespace.start:
            return False
        # maybe insert free space, if inside the disk and not at the very end
        if snippet != self.fileblocklist[-1]:
            new_free = FileSnippet(snippet.start, snippet.length, -1)
            self.fileblocklist.insert(self.fileblocklist.index(snippet), new_free)
        self.move_part(snippet, freespace, snippet.length)
        # print(f"moved {file_id}: " + str(self))
        return True


def solve():
    example = """2333133121414131402"""
    input_text = get_input_data(2024, 9, example)

    # part 1
    disk = DiskFragMap(input_text)
    while disk.has_free_space():
        disk.defrag_step()
    #print(disk)
    print(disk.checksum())

    # part 2
    disk2 = DiskFragMap(input_text)
    disk2.defrag_fullfile()
    #print(disk2)
    print(disk2.checksum())
