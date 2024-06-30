#nullable enable

using UnityEngine;
using System.Collections;
using UnityEngine.Networking;
using System;
using Newtonsoft.Json;
using System.Collections.Generic;
using UnityEditor.UI;

public class EmpireApi
{
    private string prefix;

    public EmpireApi(string prefix)
    {
        this.prefix = prefix;
    }

    public UnityWebRequest get_inventory(int id)
    {
        return UnityWebRequest.Get(prefix + "empire/" + id);
    }
}
