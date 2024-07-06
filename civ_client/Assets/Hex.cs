using System;
using UnityEngine;

public class Hex: MonoBehaviour
{
    private GameObject hexSelector;

    private Nullable<int> owner;
    private GameObject selector = null;

    public void init(GameObject hexSelector){
        this.hexSelector = hexSelector;
    }


    public void setOwner(int new_owner) {
        owner = new_owner;

        if (selector == null){
            selector = Instantiate(hexSelector, transform.position, Quaternion.Euler(90, 0, 0));
        }
    }
}
