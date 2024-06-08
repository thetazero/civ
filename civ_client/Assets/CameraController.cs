using UnityEngine;

[RequireComponent(typeof(Camera))]
[AddComponentMenu("Camera Controller")]
public class RTS_Camera : MonoBehaviour
{
    private float pan_acceleration = 40f;

    private float zoom_acceleration = 40f;
    private float max_pan_speed = 100f;
    private float max_zoom_speed = 5f;
    private float pan_damping = 100f;
    private float zoom_damping = 20f;
    private float bounding_constant = 50f;
    private Vector3 velocity;

    private float min_height = 5f;
    private float max_height = 25f;
    private float map_north_south = 100f;
    private float map_east_west = 100f;

    void Update()
    {
        float x_input = Input.GetAxis("East");
        float z_input = Input.GetAxis("North");
        float y = Input.GetAxis("Vertical");

        Vector3 pan = new Vector3(x_input, 0, z_input) * pan_acceleration * Time.deltaTime;
        Vector3 zoom = new Vector3(0, y, 0) * zoom_acceleration * Time.deltaTime;

        if (Input.GetMouseButtonDown(2))
        {
            Vector3 selected = click_position();
        }
        velocity.x *= 1f - pan_damping * Time.deltaTime;
        velocity.z *= 1f - pan_damping * Time.deltaTime;
        velocity.y *= 1f - zoom_damping * Time.deltaTime;

        velocity += pan + zoom;
        velocity.x = Mathf.Clamp(velocity.x, -max_pan_speed, max_pan_speed);
        velocity.z = Mathf.Clamp(velocity.z, -max_pan_speed, max_pan_speed);
        velocity.y = Mathf.Clamp(velocity.y, -max_zoom_speed, max_zoom_speed);
        velocity += velocity_bounds_adjust(transform.position) * Time.deltaTime * bounding_constant;

        transform.position += velocity;
        transform.rotation = rotation_from_height(transform.position.y);
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
        return Quaternion.Euler(Mathf.Lerp(35f, 75f, t), 0, 0);
    }

    Vector3 velocity_bounds_adjust(Vector3 postion)
    {
        Vector3 dv = Vector3.zero;
        dv.x -= out_of_bounds_by(postion.x, 0f, map_east_west);
        dv.z -= out_of_bounds_by(postion.z, 0f, map_north_south);
        dv.y -= out_of_bounds_by(postion.y, min_height, max_height);
        return dv;
    }

    float out_of_bounds_by(float val, float min, float max)
    {
        if (val < min)
        {
            return val - min;
        }
        else if (val > max)
        {
            return val - max;
        }
        return 0;
    }

}
