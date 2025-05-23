package de.unipassau.rustyunit;

import java.io.FileInputStream;
import java.io.IOException;
import java.util.Properties;

public class Constants {

  private static final Properties properties = loadProperties();

  public static final int CHROMOSOME_LENGTH = getInt("chromosome-length");
  public static final int INITIAL_CHROMOSOME_LENGTH = getInt("initial-chromosome-length");
  public static final double P_TEST_CHANGE = getDouble("p-test-change");
  public static final double P_TEST_INSERT = getDouble("p-test-insert");
  public static final double P_TEST_DELETE = getDouble("p-test-delete");
  public static final double P_STMT_INSERT = getDouble("p-statement-insert");
  public static final double P_CONSTANT_POOL = getDouble("p-constant-pool");
  public static final double P_CHANGE_PARAMETER = getDouble("p-change-parameter");
  public static final double P_CROSSOVER = getDouble("p-crossover");
  public static final double P_RANDOM_PERTURBATION = getDouble("p-random-perturbation");
  public static final double P_LOCAL_VARIABLES = getDouble("p-local-variables");
  public static final int POPULATION_SIZE = getInt("population-size");
  public static final int GENERATIONS = getInt("generations");
  public static final double SELECTION_BIAS = getDouble("selection-bias");
  public static final int MAX_DELTA = getInt("max-delta");
  public static final int MAX_INT = getInt("max-int");
  public static final int MAX_STRING_LENGTH = getInt("max-string-length");
  public static final String TEST_MOD_NAME = properties.getProperty("test-mod-name");
  public static final String TEST_PREFIX = properties.getProperty("test-prefix");
  public static final int TEST_TIMEOUT = getInt("test-timeout");

  public static final String DB_NAME = properties.getProperty("db-name");
  public static final String DB_USER = properties.getProperty("db-user");
  public static final String DB_PASSWORD = properties.getProperty("db-password");


  private static Properties loadProperties() {
    var properties = new Properties();
    try {
      properties.load(Thread.currentThread().getContextClassLoader().getResourceAsStream("config.properties"));
    } catch (IOException e) {
      e.printStackTrace();
      System.exit(1);
    }

    return properties;
  }

  private static int getInt(String name) {
    return Integer.parseInt(properties.getProperty(name));
  }

  private static double getDouble(String name) {
    return Double.parseDouble(properties.getProperty(name));
  }
}
