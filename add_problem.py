#! /usr/bin/env python3

from bs4 import BeautifulSoup
import sys
from urllib import request


def refine_title(title):
    """
Convert the title of a Project Euler problem page to something which can be
used as the name of a Rust source file.

:param title: Webpage title.

:return: Rust module name derived from the title.
    """
    title = title[title.index(' ') + 1 :].removesuffix(' - Project Euler').lower()
    refined_title = []
    for char in title:
        if char.isalnum():
            refined_title.append(char)
        elif char == ' ':
            refined_title.append('_')
    return ''.join(refined_title)


def main():
    """
Main function.
    """
    problem = int(sys.argv[1])
    url = f'https://projecteuler.net/problem={problem}'
    html = request.urlopen(url).read()
    soup = BeautifulSoup(html, 'html.parser')
    title = refine_title(soup.title.string)

    with open(f'src/solutions/{title}.rs', 'x') as solution:
        print('pub fn solve()->i64{0}', file=solution)
    with open('src/solutions.rs', 'a') as solutions:
        print(f'pub mod {title};', file=solutions)
    with open('README.md', 'a') as readme:
        print(f'|[{problem}]({url})|[`{title}.rs`](src/solutions/{title}.rs)|', file=readme)
    with open('src/main.rs', 'a') as main_rs:
        print(f'        {problem} => solutions::{title}::solve,', file=main_rs)


if __name__ == '__main__':
    main()
