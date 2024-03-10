import oblivion_rs
import pytest


@pytest.mark.asyncio
async def test_async_get():
    (await oblivion_rs.async_get("127.0.0.1:813")).text()


def test_get():
    oblivion_rs.get("127.0.0.1:813").text()


@pytest.mark.asyncio
async def test_async_post():
    (await oblivion_rs.async_post("127.0.0.1:813", {"test": "test"})).text()


def test_post():
    oblivion_rs.post("127.0.0.1:813", {"test": "test"}).text()


@pytest.mark.asyncio
async def test_async_put():
    (await oblivion_rs.async_put("127.0.0.1:813", {"test": "test"}, b"dfwef432r43")).text()


def test_get():
    oblivion_rs.put("127.0.0.1:813", {"test": "test"}, b"dfwef432r43").text()
