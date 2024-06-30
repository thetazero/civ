using System;
using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEditor.EditorTools;
using UnityEngine;

public class Game : MonoBehaviour
{

    public GameObject tileDesert;
    public GameObject tileForest;
    public GameObject tileMountain;
    public GameObject tileSnowyMountain;
    public GameObject tileShallows;
    public GameObject tileOcean;
    public GameObject tileBeach;


    public GameObject buildingCity;

    private ServerIO serverIO = new ServerIO();
    private float hex_size = 1.0f;
    // Start is called before the first frame update

    private int empire_id = 0;
    void Start()
    {
        StartCoroutine(serverIO.loadAllHex(
            (List<IndexedHex> data) =>
            {
                foreach (IndexedHex hex in data)
                {
                    // Debug.Log("HexData: " + hex.idx.row + " " + hex.idx.col + " " + hex.tile.kind);
                    GameObject hexObj = SpawnTile(hex.tile.kind, hex.idx);
                    if (hex.tile.building != null)
                    {
                        SpawnBuilding(hex.tile.building, hexObj, hex.tile.owner);
                    }
                }
            }
        ));
    }

    GameObject SpawnBuilding(BuildingData data, GameObject parentHex, Nullable<int> owner)
    {

        Debug.Log("Building: " + data.kind);

        GameObject buildingObj = null;
        switch (data.kind)
        {
            case BuildingKind.Capital:
                buildingObj = Instantiate(buildingCity, parentHex.transform.position, Quaternion.Euler(90, 0, 0), parentHex.transform);
                break;
        }
        buildingObj.transform.localScale = new Vector3(0.25f, 0.25f, 0.25f);

        Material mat = buildingObj.GetComponent<Renderer>().material;

        if (owner != null) mat.color = Color.HSVToRGB(owner.GetValueOrDefault() / 10.0f, 1.0f, 1.0f);
        else mat.color = Color.magenta;

        return buildingObj;
    }

    GameObject SpawnTile(TileKind kind, HexIndex idx)
    {

        GameObject hexObj = null;
        if (kind == TileKind.Desert && tileDesert != null)
        {
            hexObj = tileDesert;
        }
        else if (kind == TileKind.Forest && tileForest != null)
        {
            hexObj = tileForest;
        }
        else if (kind == TileKind.Mountain && tileMountain != null)
        {
            hexObj = tileMountain;
        }
        else if (kind == TileKind.SnowyMountain && tileSnowyMountain != null)
        {
            hexObj = tileSnowyMountain;
        }
        else if (kind == TileKind.Shallows && tileShallows != null)
        {
            hexObj = tileShallows;
        }
        else if (kind == TileKind.Ocean && tileOcean != null)
        {
            hexObj = tileOcean;
        }
        else if (kind == TileKind.Beach && tileBeach != null)
        {
            hexObj = tileBeach;
        }
        else
        {
            Debug.Log("Unknown tile kind: " + kind);
        }

        if (hexObj != null)
        {
            hexObj = Instantiate(hexObj);
            hexObj.transform.position = hex_to_location(idx.row, idx.col, hex_size);
        }

        return hexObj;
    }

    Vector3 hex_to_location(int row, int col, float hex_size)
    {
        float x = hex_size * Mathf.Sqrt(3f) * (col + 0.5f * (row & 1));
        float z = hex_size * 3f / 2f * row;
        return new Vector3(x, 0f, z);
    }

    // Update is called once per frame
    void Update()
    {

    }
}
