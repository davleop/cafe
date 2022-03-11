#!/usr/bin/env python

import json
from websocket import create_connection

def build_json_res(t: int, msg: str) -> str:
    return json.dumps({'type':t, 'msg':msg})


def main():
    ws = create_connection('ws://localhost:42069')

    # send two ready json requests
    ready_req = build_json_res(0, '')

    print('Ready #1...')
    ws.send(ready_req)
    print(ws.recv())
    #print('Ready #2...')
    #ws.send(ready_req)
    #print(ws.recv())

    # heartbeat
    print('Heartbeat...')
    heartbeat_req = build_json_res(1, '')
    ws.send(heartbeat_req)
    print(ws.recv())

    # message
    print('Message...')
    message_req = build_json_res(2, 'test message')
    ws.send(message_req)
    print(ws.recv())

    # debug message
    print('Debug...')
    debug_req = build_json_res(3, 'debug message')
    ws.send(debug_req)
    print(ws.recv())

    # end connection
    print('End...')
    end_req = build_json_res(4, 'goodbye')
    ws.send(end_req)
    ws.close()

    print('Completed all transactions')


if __name__ == '__main__':
    main()
