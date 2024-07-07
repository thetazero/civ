using UnityEngine;

[RequireComponent(typeof(Camera))]
[AddComponentMenu("Camera Controller")]
public class RTS_Camera : MonoBehaviour
{
    private float pan_acceleration = 200f;

    private float zoom_acceleration = 150f;
    private float max_pan_speed = 0.3f;
    private float max_zoom_speed = 0.7f;
    private float pan_damping = 20f;
    private Vector3 velocity;

    private float min_height = 5f;
    private float max_height = 25f;
    public float map_north_south = 100f;
    public float map_east_west = 100f;

    void Update()
    {
        float x_input = Input.GetAxis("East");
        float z_input = Input.GetAxis("North");
        float y = Input.GetAxis("Vertical");

        Vector3 pan = new Vector3(x_input, 0, z_input) * pan_acceleration * Time.deltaTime;
        float zoom = y * zoom_acceleration * Time.deltaTime;

        if (Input.GetMouseButtonDown(2))
        {
            Vector3 selected = click_position();
        }
        velocity += pan;

        Vector3 pan_component = new Vector3(velocity.x, 0, velocity.z);
        pan_component *= Mathf.Max(0f, 1f - pan_damping * Time.deltaTime); // drag
        pan_component = Vector3.ClampMagnitude(pan_component, max_pan_speed);

        velocity.x = pan_component.x;
        velocity.z = pan_component.z;
        velocity.y = Mathf.Clamp(zoom, -max_zoom_speed, max_zoom_speed);

        transform.position += velocity;
        transform.rotation = rotation_from_height(transform.position.y);
        apply_bounding_box();
    }


    Vector3 click_position()
    {
        Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
        RaycastHit hit;
        if (Physics.Raycast(ray, out hit, Mathf.Infinity))
        {
            return new Vector3(hit.point.x, Mathf.Max(hit.point.y, 0), hit.point.z);
        }
        return Vector3.zero;
    }

    Quaternion rotation_from_height(float height)
    {
        height = Mathf.Clamp(height, min_height, max_height);
        float t = (height - min_height) / (max_height - min_height);
        return Quaternion.Euler(Mathf.Lerp(37f, 80f, t), 0, 0);
    }


    void apply_bounding_box()
    {
        Vector3 position = transform.position;  
        if (position.x < 0f) { position.x = 0f; velocity.x = Mathf.Max(0f, velocity.x); }
        if (position.x > map_east_west) { position.x = map_east_west; velocity.x = Mathf.Min(0f, velocity.x); }

        if (position.z < 0f) { position.z = 0f; velocity.z = Mathf.Max(0f, velocity.z); }
        if (position.z > map_north_south) { position.z = map_north_south; velocity.z = Mathf.Min(0f, velocity.z); }

        if (position.y < min_height) { position.y = min_height; velocity.y = Mathf.Max(0f, velocity.y); }
        if (position.y > max_height) { position.y = max_height; velocity.y = Mathf.Min(0f, velocity.y); }

        Debug.Log(velocity.x);

        transform.position = position;
    }

}
