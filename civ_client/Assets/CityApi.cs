#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;
using UnityEditor.UI;

public class CityApi
{
    private string prefix;

    public CityApi(string prefix)
    {
        this.prefix = prefix;
    }

    public UnityWebRequest get_all()
    {
        return UnityWebRequest.Get(prefix + "city/all/");
    }

    public UnityWebRequest get_by_id(uint id)
    {
        return UnityWebRequest.Get(prefix + "city/" + id);
    }
}
