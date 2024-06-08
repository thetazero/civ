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
            (IndexedHex[] data) =>
            {
                Debug.Log("Recieved data: " + data);
                foreach (IndexedHex hex in data)
                {
                    // Debug.Log("HexData: " + hex.idx.row + " " + hex.idx.col + " " + hex.tile.kind);
                    GameObject hexObj = null;
                    if (hex.tile.kind == "Desert" && tileDesert != null)
                    {
                        hexObj = tileDesert;
                    } else if (hex.tile.kind == "Forest" && tileForest != null)
                    {
                        hexObj = tileForest;
                    } else if (hex.tile.kind == "Mountain" && tileMountain != null)
                    {
                        hexObj = tileMountain;
                    } else if (hex.tile.kind == "SnowyMountain" && tileSnowyMountain != null)
                    {
                        hexObj = tileSnowyMountain;
                    } else if (hex.tile.kind == "Shallows" && tileShallows != null)
                    {
                        hexObj = tileShallows;
                    } else if (hex.tile.kind == "Ocean" && tileOcean != null)
                    {
                        hexObj = tileOcean;
                    } else if (hex.tile.kind == "Beach" && tileBeach != null)
                    {
                        hexObj = tileBeach;
                    } else {
                        Debug.Log("Unknown tile kind: " + hex.tile.kind);
                    }

                    if (hexObj != null)
                    {
                        hexObj = Instantiate(hexObj);
                        hexObj.transform.position = hex_to_location(hex.idx.row, hex.idx.col, hex_size);
                    }
                }
            }
        ));
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
