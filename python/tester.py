#!/usr/bin/env python

import json
import asyncio

from connsocket import Socket

class Tester:
    def __init__(self, uri: str):
        self.sock = Socket(uri)

    async def build_json_res(self, t: int, msg: str) -> str:
        return json.dumps({'type':t, 'msg':msg})

    async def test_ready(self):
        res = await self.build_json_res(0, '')
        await self.sock.send(res)
        return await self.sock.recv()

    async def test_heartbeat(self):
        res = await self.build_json_res(1, '')
        await self.sock.send(res)
        return await self.sock.recv()

    async def test_message(self, msg: str):
        res = await self.build_json_res(2, msg)
        await self.sock.send(res)
        return await self.sock.recv()

    async def test_debug(self, msg: str):
        res = await self.build_json_res(3, msg)
        await self.sock.send(res)
        return await self.sock.recv()

    async def test_end(self):
        res = await self.build_json_res(4, '')
        await self.sock.send(res)
        await self.sock.close()

    async def go(self) -> list[bool]:
        ret = list() # list of bools

        # two ready's because it goes through two different handlers
        tr1 = await self.test_ready()
        ret.append(tr1)
        tr2 = await self.test_ready()
        ret.append(tr2)


        await self.test_end()

        return ret

