from typing import Callable, Dict, Any
from bustapi import _core

class BustAPI:
    def __init__(self):
        self._app = _core.PyApp()

    def route(self, path: str, method: str):
        def decorator(fn: Callable[[Dict[str, Any]], Any]):
            self._app.add_route(method, path, fn)
            return fn
        return decorator

    def get(self, path: str): return self.route(path, "GET")
    def post(self, path: str): return self.route(path, "POST")
    def put(self, path: str): return self.route(path, "PUT")
    def delete(self, path: str): return self.route(path, "DELETE")
    def patch(self, path: str): return self.route(path, "PATCH")

    def run(self, host: str = "127.0.0.1", port: int = 8000):
        self._app.run(host, port)

_app_singleton = BustAPI()

def get(path: str): return _app_singleton.get(path)
def post(path: str): return _app_singleton.post(path)
def put(path: str): return _app_singleton.put(path)
def delete(path: str): return _app_singleton.delete(path)
def patch(path: str): return _app_singleton.patch(path)

def run(host: str = "127.0.0.1", port: int = 8000):
    _app_sin