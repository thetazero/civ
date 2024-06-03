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
        // StartCoroutine(serverIO.LoadAll(
        //  (GameData data) =>
        //  {
        //      Debug.Log("Recieved data: " + data);
        //      WorldMap map = data.world_state.map;
        //      for (int i = 0; i < map.rows; i++)
        //      {
        //          for (int j = 0; j < map.cols; j++)
        //          {
        //              Debug.Log("HexData: " + i + " " + j + " " + map.data[i][j].kind);
        //              HexData hexData = map.data[i][j];
        //              GameObject hex = Instantiate(hexPrefab);
        //              hex.transform.position = hex_to_location(i, j, hex_size);
        //          }
        //      }
        //  }
        // ));
        StartCoroutine(serverIO.loadHex(
            0, 0,
            (HexData data) =>
            {
                Debug.Log("Recieved data: " + data.kind);
                GameObject hex = Instantiate(hexPrefab);
                hex.transform.position = hex_to_location(0, 0, hex_size);
            }
        ));
    }

    Vector3 hex_to_location(int row, int col, float hex_size)
    {
        float x = hex_size * Mathf.Sqrt(3f) * (col + 0.5f * (row & 1));
        float y = hex_size * 3f / 2f * row;
        return new Vector3(x, y, 0f);
    }

    // Update is called once per frame
    void Update()
    {

    }
}
