#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;


public class ServerIO
{


    public IEnumerator loadHex(
        int row, int col,
        Action<TileData> callback
    )
    {
        UnityWebRequest req = TileApi.get(row, col);
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
        UnityWebRequest req = TileApi.get_all();
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

    [JsonProperty("owner")]
    public Nullable<int> owner;

    [JsonProperty("building")]
    public BuildingData? building;
}


public class BuildingData
{
    [JsonProperty("kind")]
    public BuildingKind kind;
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
