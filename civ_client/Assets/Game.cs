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

    private ServerIO serverIO = new ServerIO();
    private float hex_size = 1.0f;
    // Start is called before the first frame update
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
                        BuildingData buildingData = hex.tile.building;
                        Debug.Log("Building: " + buildingData.kind);
                        Debug.Log("Building owner: " + buildingData.owner);
                    }
                }
            }
        ));
    }

    GameObject SpawnTile(String kind, HexIndex idx)
    {

        GameObject hexObj = null;
        if (kind == "Desert" && tileDesert != null)
        {
            hexObj = tileDesert;
        }
        else if (kind == "Forest" && tileForest != null)
        {
            hexObj = tileForest;
        }
        else if (kind == "Mountain" && tileMountain != null)
        {
            hexObj = tileMountain;
        }
        else if (kind == "SnowyMountain" && tileSnowyMountain != null)
        {
            hexObj = tileSnowyMountain;
        }
        else if (kind == "Shallows" && tileShallows != null)
        {
            hexObj = tileShallows;
        }
        else if (kind == "Ocean" && tileOcean != null)
        {
            hexObj = tileOcean;
        }
        else if (kind == "Beach" && tileBeach != null)
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
