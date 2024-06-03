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
        Action<HexData> callback
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
            HexData data = JsonUtility.FromJson<HexData>(req.downloadHandler.text);
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
    public HexData[][] data;
    public int rows;
    public int cols;
}

[System.Serializable]
public class HexData
{
    public string kind;
    public BuildingData building;
}

[System.Serializable]
public class BuildingData
{

}
