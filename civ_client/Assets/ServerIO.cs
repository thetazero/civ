#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;


public class ServerIO
{
    private static string api_root = "http://127.0.0.1:8000/";
    private static TileApi tileApi = new TileApi(api_root);
    private static CityApi cityApi = new CityApi(api_root);

    public IEnumerator loadHex(
        int row, int col,
        Action<TileData> callback
    )
    {
        UnityWebRequest req = tileApi.get(row, col);
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
        UnityWebRequest req = tileApi.get_all();
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

    public IEnumerator loadAllCity(
        Action<Dictionary<int, City>> callback
    ) {

        UnityWebRequest req = cityApi.get_all();
        yield return req.SendWebRequest();
        if (req.result != UnityWebRequest.Result.Success)
        {
            Debug.Log(req.error);
        }
        else
        {
            Debug.Log("Received: " + req.downloadHandler.text);
            Dictionary<int, City> data = JsonConvert.DeserializeObject<Dictionary<int, City>>(req.downloadHandler.text);
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

    [JsonProperty("city")]
    public Nullable<int> city;

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

    public override int GetHashCode()
    {
        return (row, col).GetHashCode();
    }

    public override bool Equals(object obj)
    {
        if (obj is HexIndex other)
        {
            return this.row == other.row && this.col == other.col;
        }
        return false;
    }
}


public enum BuildingKind
{
    Capital
}

public class City {
    [JsonProperty("owner")]
    public int owner;

    [JsonProperty("tiles")]
    public List<HexIndex> tiles;
}
