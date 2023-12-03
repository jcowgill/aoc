import itertools
from pathlib import Path
from typing import Generic, Iterator, Optional, Tuple, TypeVar


T = TypeVar("T")


class Tag:
    resolves: Optional[str]

    def __init__(self, init: Optional[str] = None) -> None:
        self.resolved = init

    def __str__(self) -> str:
        if self.resolved:
            return self.resolved
        raise RuntimeError("attempt to use tag before resolution")


class NameCache(Generic[T]):
    names: dict[str, list[Tuple[Tag, T]]]

    def __init__(self) -> None:
        self.names = {}

    def register(self, name: str, num_prefix: str, value: T) -> Tag:
        if name.isdigit():
            name = num_prefix + name

        tag = Tag()
        if name in self.names:
            self.names[name].append((tag, value))
        else:
            self.names[name] = [(tag, value)]
        return tag

    def __iter__(self) -> Iterator[Tuple[Tag, T]]:
        return itertools.chain.from_iterable(self.names.values())

    def resolve(self) -> None:
        for k, v in self.names.items():
            if len(v) == 1:
                # No need to change name
                v[0][0].resolved = k
            else:
                # Prefer numbers unless name ends with a number already
                if not k[-1].isdigit():
                    letter = "1"
                elif k[0].isupper():
                    letter = "A"
                else:
                    letter = "a"

                # Use different letters to disambiguate each value
                for tag, _ in v:
                    tag.resolved = k + letter
                    letter = chr(ord(letter) + 1)


class StringCache:
    def __init__(self) -> None:
        self.nc = NameCache[str]()

    def register(self, name: str, is_output: bool, path: Path) -> Tag:
        data = path.read_text().rstrip()

        # Is this already registered?
        for k, v in self.nc:
            if data == v:
                return k

        if len(data) >= 100 or "\n" in data:
            # Register this as const input
            num_prefix = "OUT" if is_output else "IN"
            return self.nc.register(name.upper(), num_prefix, data)
        elif '"' in data:
            # Use Rust raw string
            assert '"#' not in data
            return Tag('r#"' + data + '"#')
        else:
            # Use normal string
            return Tag('"' + data + '"')

    def resolve(self) -> None:
        self.nc.resolve()

    def dump(self) -> str:
        result = ""
        for k, v in self.nc:
            result += f"\n\n    const {k}: &str = indoc!{{"
            raw_str = '"' in v
            if raw_str:
                assert '"#' not in v
                result += 'r#"\n'
            else:
                result += '"\n'

            for line in v.splitlines():
                line = line.rstrip()
                if line:
                    result += " " * 8 + line + "\n"
                else:
                    result += "\n"

            if raw_str:
                result += '    "#};'
            else:
                result += '    "};'
        return result


def gen_day(data_dir: Path) -> str:
    # Load all input data
    str_cache = StringCache()
    inputs = {}
    children = list(sorted(data_dir.iterdir()))

    for path in children:
        if path.name.endswith(".in.1"):
            name = path.name.removesuffix(".in.1")
            reg = [1]
        elif path.name.endswith(".in.2"):
            name = path.name.removesuffix(".in.2")
            reg = [2]
        elif path.name.endswith(".in"):
            name = path.name.removesuffix(".in")
            reg = [1, 2]
        elif "in" in path.name:
            raise RuntimeError(f"Input WTF? {path}")
        else:
            # Not an input
            continue

        value = str_cache.register(name, False, path)
        for r in reg:
            inputs[(name, r)] = value

    # Compute all test lines
    test_names = NameCache[Tuple[int, Tag, Tag]]()
    for path in children:
        if path.name.endswith(".out.1"):
            test = (path.name.removesuffix(".out.1"), 1)
        elif path.name.endswith(".out.2"):
            test = (path.name.removesuffix(".out.2"), 2)
        elif "out" in path.name:
            raise RuntimeError(f"Output WTF? {path}")
        else:
            continue

        value = str_cache.register(test[0], True, path)
        test_names.register(test[0].lower(), "example", (test[1], inputs[test], value))

    str_cache.resolve()
    test_names.resolve()

    def dump_test(name: Tag, value: Tuple[int, Tag, Tag]) -> str:
        star, inp, out = value
        return f"    star_test!({name}, star{star}, {inp}, {out});"

    star1 = [dump_test(k, v) for k, v in test_names if v[0] == 1]
    star1_str = "\n\n" + "\n".join(star1) if star1 else ""
    star2 = [dump_test(k, v) for k, v in test_names if v[0] == 2]
    star2_str = "\n\n" + "\n".join(star2) if star2 else ""
    cache_dump = str_cache.dump()
    indoc = "\n    use indoc::indoc;" if cache_dump else ""

    return (
        "\n"
        "#[cfg(test)]\n"
        "mod test {\n"
        f"    use super::*;{indoc}{star1_str}{star2_str}{cache_dump}\n"
        "}\n"
    )


def process_day(data_dir: Path):
    # Find source path
    src_file = (
        Path("src")
        / ("yr" + str(data_dir.parent.name))
        / ("day" + str(int(str(data_dir.name))) + ".rs")
    )
    if not src_file.exists():
        raise RuntimeError(f"could not find {src_file}")

    # Generate date
    to_append = gen_day(data_dir)

    # Append to source
    with src_file.open("a") as f:
        f.write(to_append)


for year in Path("tests/data").iterdir():
    for day in year.iterdir():
        print(day)
        process_day(day)
