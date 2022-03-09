#!/usr/bin/env python

import asyncio

from tester import Tester

async def main():
    tester = Tester('ws://localhost:42069')
    res = await tester.go()

    print(res)

if __name__ == '__main__':
    asyncio.run(main())
