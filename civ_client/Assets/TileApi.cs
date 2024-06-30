#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;
using UnityEditor.UI;

public class TileApi
{
    private string prefix;

    public TileApi(string prefix)
    {
        this.prefix = prefix;
    }

    public UnityWebRequest get_all()
    {
        return UnityWebRequest.Get(prefix + "tile/all/");
    }

    public UnityWebRequest get(int row, int col)
    {
        return UnityWebRequest.Get(prefix + "tile/row/" + row + "/col/" + col);
    }
}
