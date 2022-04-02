#!/usr/bin/env python

import asyncio
import websockets


class Socket:
    def __init__(self, uri: str):
        self.uri = uri
        self.conn = None
        self.loop = asyncio.get_event_loop()

    async def send(self, msg: str):
        if self.conn is None:
            self.conn = await websockets.connect(self.uri)
        await self.conn.send(msg)

    async def recv(self) -> str:
        return await self.conn.recv()

    async def close(self):
        await self.conn.close()

