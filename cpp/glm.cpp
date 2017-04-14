#include <iostream>
#include <glm/glm.hpp>
#include <glm/gtx/rotate_vector.hpp>
#include <glm/gtc/matrix_inverse.hpp>
#include <glm/gtx/string_cast.hpp>

int main(int argc, char *argv[]) {
    float camera_z = 10.0;
    unsigned int width = 640;
    unsigned int height = 480;

    glm::mat4 projection_matrix = glm::perspective(glm::radians(45.0), (double)width / (double)height,
                                                  camera_z / 10.0, camera_z * 10.0);

    glm::mat4 view_matrix = glm::lookAt(glm::vec3(0.0, 0.0, camera_z), glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 1.0, 0.0));

    std::cout << "proj: " << glm::to_string(projection_matrix) << std::endl;
    std::cout << "view: " << glm::to_string(view_matrix) << std::endl;

    glm::vec4 viewport = glm::vec4(0.0f, 0.0f, (float)width, (float)height);

    glm::mat4 foo_one = projection_matrix * view_matrix;
    glm::mat4 foo_two = glm::inverse(foo_one);

    std::cout << "foo1:" << glm::to_string(glm::transpose(foo_one)) << std::endl;
    std::cout << "foo2:" << glm::to_string(glm::transpose(foo_two)) << std::endl;

    glm::vec3 ray_start = glm::unProject(glm::vec3(100.0, 200.0, 0.0f), view_matrix, projection_matrix, viewport);

    std::cout << "unProject:" << glm::to_string(ray_start) << std::endl;

    glm::mat4 translation = glm::translate(glm::vec3(0.0f, -1.0f, 0.0f));
    std::cout << "translation: " << glm::to_string(translation) << std::endl;
}
