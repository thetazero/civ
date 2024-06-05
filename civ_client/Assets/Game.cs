using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Game : MonoBehaviour
{

    public GameObject hexPrefab;
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
                    Debug.Log("HexData: " + hex.idx.row + " " + hex.idx.col + " " + hex.tile.kind);
                    GameObject hexObj = Instantiate(hexPrefab, transform);
                    hexObj.transform.position = hex_to_location(hex.idx.row, hex.idx.col, hex_size);
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
