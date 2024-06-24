#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;


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
        Action<List<IndexedHex>> callback
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
            List<IndexedHex> data = JsonConvert.DeserializeObject<List<IndexedHex>>(req.downloadHandler.text);
            callback(data);
        }
    }
}

public class GameData
{
    [JsonProperty("world_state")]
    public WorldState world_state;
}

public class WorldState
{
    [JsonProperty("map")]
    public WorldMap map;
}

public class WorldMap
{
    [JsonProperty("data")]
    public IndexedHex[] data;

    [JsonProperty("rows")]
    public int rows;

    [JsonProperty("cols")]
    public int cols;
}

public class IndexedHex
{
    [JsonProperty("idx")]
    public HexIndex idx;

    [JsonProperty("tile")]
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

public class TileData
{
    [JsonProperty("kind")]
    public TileKind kind;

    [JsonProperty("building")]
    public BuildingData? building;
}


public class BuildingData
{
    [JsonProperty("kind")]
    public BuildingKind kind;
    [JsonProperty("owner")]
    public int owner;
}

public class HexIndex
{
    [JsonProperty("row")]
    public int row;
    [JsonProperty("col")]
    public int col;
}


public enum BuildingKind
{
    Capital
}
