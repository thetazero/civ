#nullable enable
using System;
using UnityEngine;
using UnityEngine.Assertions;

public class Hex : MonoBehaviour
{
    private static GameObject selectorPrefab;
    private static GameObject buildingCityPrefab;

    private Nullable<int> owner;


    // Children
    private GameObject selector = null;
    private GameObject building = null;

    public static void init(GameObject selectorPrefab, GameObject buildingCityPrefab)
    {
        Hex.selectorPrefab = selectorPrefab;
        Hex.buildingCityPrefab = buildingCityPrefab;
    }


    public void setOwner(int new_owner)
    {
        owner = new_owner;

        if (selector == null)
        {
            selector = Instantiate(selectorPrefab, transform.position, Quaternion.Euler(90, 0, 0));
            Color color = Color.HSVToRGB(ownerHue(owner.Value), 1.0f, 1.0f);
            color.a = 0.3f;
            setColor(selector, color);
        }
    }

    public void setBuilding(BuildingData data)
    {
        Assert.IsTrue(owner.HasValue);

        GameObject buildingObj = null;
        switch (data.kind)
        {
            case BuildingKind.Capital:
                buildingObj = Instantiate(buildingCityPrefab, this.transform.position, Quaternion.Euler(90, 0, 0), this.transform);
                break;
            default:
                Assert.IsFalse(true);
                break;
        }
        building = buildingObj;

        building.transform.localScale = new Vector3(0.25f, 0.25f, 0.25f);

        Color hexColor = Color.magenta;
        if (owner.HasValue) hexColor = Color.HSVToRGB(ownerHue(owner.Value), 1.0f, 1.0f);

        setColor(building, hexColor);
    }

    public static void setColor(GameObject obj, Color color)
    {
        Material mat = obj.GetComponent<Renderer>().material;
        mat.color = color;
    }

    private static float ownerHue(int owner)
    {
        return owner / 10.0f;
    }

}
