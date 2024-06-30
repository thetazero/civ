#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;

public class TileApi
{
    private static string prefix = "http://127.0.0.1:8000/";

    public static UnityWebRequest get_all() {
        return UnityWebRequest.Get(prefix + "tile/all/");
    }

    public static UnityWebRequest get(int row, int col) {
        return UnityWebRequest.Get(prefix + "tile/row/" + row + "/col/" + col);
    }
}
