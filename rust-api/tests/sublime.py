
Point = int

class Region:
    def __init__(self, a: int, b: int) -> None:
        self.a = Point(a)
        self.b = Point(b)

    def to_tuple(self):
        return (self.a, self.b)

    def __str__(self) -> str:
        return self.to_tuple().__str__()

class Selection:
    inner = []

    def __init__(self) -> None:
        pass

    def add(self, region):
        self.inner.append(region)


    def __len__(self) -> int:
        return self.inner.__len__()

    def __getitem__(self, index: int):
        return self.inner.__getitem__(index)

    def __str__(self) -> str:
        string = "["
        for region in self.inner:
            string += f"{region.__str__()}, "
        string += "]"
        return string



class View:
    selection = Selection()
    def sel(self) -> Selection:
        return self.selection

    def run_command(self, cmd: str, args):
        print(f"command to run: {cmd} with args: {args}")
