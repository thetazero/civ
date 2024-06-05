using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;

public class ServerIO
{
    private static string prefix = "http://127.0.0.1:8000/";
    public IEnumerator LoadAll(
        Action<GameData> callback
    )
    {
        UnityWebRequest www = UnityWebRequest.Get(prefix + "game/all");
        Debug.Log("Sending request to: " + prefix + "game/all");
        yield return www.SendWebRequest();
        if (www.result != UnityWebRequest.Result.Success)
        {
            Debug.Log(www.error);
        }
        else
        {
            Debug.Log("Received: " + www.downloadHandler.text);
            GameData data = JsonUtility.FromJson<GameData>(www.downloadHandler.text);
            callback(data);
        }
    }

    public IEnumerator loadHex(
        int row, int col,
        Action<TileData> callback
    )
    {
        UnityWebRequest req = UnityWebRequest.Get(prefix + "game/tile/" + row + "/" + col);
        yield return req.SendWebRequest();
        if (req.result != UnityWebRequest.Result.Success)
        {
            Debug.Log(req.error);
        }
        else
        {
            Debug.Log("Received: " + req.downloadHandler.text);
            TileData data = JsonUtility.FromJson<TileData>(req.downloadHandler.text);
            callback(data);
        }
    }

    public IEnumerator loadAllHex(
        Action<IndexedHex[]> callback
    )
    {
        UnityWebRequest req = UnityWebRequest.Get(prefix + "game/tile/all");
        yield return req.SendWebRequest();
        if (req.result != UnityWebRequest.Result.Success)
        {
            Debug.Log(req.error);
        }
        else
        {
            Debug.Log("Received: " + req.downloadHandler.text);
            IndexedHex[] data = JsonUtility.FromJson<Stupid<IndexedHex[]>>(req.downloadHandler.text).data;
            callback(data);
        }
    }
}

[System.Serializable]
public class GameData
{
    public WorldState world_state;
}

[System.Serializable]
public class WorldState
{
    public WorldMap map;
}

[System.Serializable]
public class WorldMap
{
    public IndexedHex[] data;
    public int rows;
    public int cols;
}

[System.Serializable]
public class IndexedHex
{
    public HexIndex idx;
    public TileData tile;
}

public enum TileKind
{
    Desert,
    Forest,
    Mountain,
    SnowyMountain,
    Shallows,
    Ocean,
    Beach,
    Unknown,
}

[System.Serializable]
public class TileData
{
    public string kind;
    public BuildingData building;
}


[System.Serializable]
public class BuildingData
{

}

[System.Serializable]
public class HexIndex
{
    public int row;
    public int col;
}

[System.Serializable]
public class Stupid<T>
{
    public T data;
}
